mod dev_ver_01;
mod dev_ver_02;
mod dev_ver_03;

use dev_ver_01::run_version_01;
use dev_ver_02::run_version_02;
use dev_ver_03::run_version_03;

fn main() {
    run_version_01(false);
    run_version_02(false);
    run_version_03(false);
}
