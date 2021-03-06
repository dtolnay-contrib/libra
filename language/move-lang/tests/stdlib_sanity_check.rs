// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use move_lang::{move_compile_no_report, shared::Address};
use std::{fs, path::Path};

use move_lang::test_utils::*;

const OUT_EXT: &str = "out";

const KEEP_TMP: &str = "KEEP";

// Runs all tests under the test/testsuite directory.
fn sanity_check_testsuite_impl(
    path: &Path,
    std_lib_is_target: bool,
    std_lib_dir: String,
) -> datatest_stable::Result<()> {
    let mut targets: Vec<String> = vec![path.to_str().unwrap().to_owned()];
    let mut deps: Vec<String> = vec![];
    if std_lib_is_target {
        targets.push(std_lib_dir)
    } else {
        deps.push(std_lib_dir)
    }
    let sender = Some(Address::LIBRA_CORE);

    let out_path = path.with_extension(OUT_EXT);

    let (files, units_or_errors) = move_compile_no_report(&targets, &deps, sender, None)?;
    let errors = match units_or_errors {
        Err(errors) => errors,
        Ok(units) => move_lang::compiled_unit::verify_units(units).1,
    };
    let has_errors = !errors.is_empty();
    let error_buffer = if has_errors {
        move_lang::errors::report_errors_to_buffer(files, errors)
    } else {
        vec![]
    };

    let save_errors = read_bool_var(KEEP_TMP);

    fs::write(out_path.clone(), error_buffer)?;
    let rendered_errors = fs::read_to_string(out_path.clone())?;
    if !save_errors {
        fs::remove_file(out_path)?;
    }

    if has_errors {
        let msg = format!("Expected success. Unexpected errors:\n{}", rendered_errors);
        error(msg)
    } else {
        Ok(())
    }
}

fn sanity_check_testsuite(path: &Path) -> datatest_stable::Result<()> {
    sanity_check_testsuite_impl(path, true, STD_LIB_DIR.to_string())?;
    sanity_check_testsuite_impl(path, false, STD_LIB_COMPILED_DIR.to_string())
}

datatest_stable::harness!(
    sanity_check_testsuite,
    STD_LIB_TRANSACTION_SCRIPTS_DIR,
    r".*\.move"
);
