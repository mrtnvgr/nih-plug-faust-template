# NIH-plug with rust-faust template

## TODO

- [x] fix "NotFound" faust compilation error
- [x] process samples through dsp directly
- [ ] params should be taken from generated dsp struct (?)
- [ ] mono/stereo stuff (don't forget vst/clap features)
- …
- [ ] Start preparing for v0.1
- [ ] Squash commits, remove TODO list

This template can be used with
[cookiecutter](https://github.com/cookiecutter/cookiecutter) to create a new
[NIH-plug](https://github.com/robbert-vdh/nih-plug) with [rust-faust](https://github.com/Frando/rust-faust/) project:

```bash
cookiecutter gh:mrtnvgr/nih-plug-faust-template
```

Check out the generated project's readme for building instructions.
