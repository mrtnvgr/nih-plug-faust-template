fn main() {
    println!("cargo:rerun-if-changed=dsp");
    faust_build::build_dsp("dsp/{{ cookiecutter.dsp_filename }}.dsp");
}
