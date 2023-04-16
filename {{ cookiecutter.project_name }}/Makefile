all:
	cargo xtask bundle {{ cookiecutter.project_name }} --release

install:
	cp target/bundled/{{ cookiecutter.struct_name }}.clap ~/.clap/

uninstall:
	rm -f ~/.clap/{{ cookiecutter.struct_name }}.clap
