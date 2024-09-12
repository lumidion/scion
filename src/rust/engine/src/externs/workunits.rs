// Copyright 2021 Pants project contributors (see CONTRIBUTORS.md).
// Licensed under the Apache License, Version 2.0 (see LICENSE).

use workunit_store::Metric;

pub fn all_counter_names() -> Vec<String> {
    Metric::all_metrics()
}
