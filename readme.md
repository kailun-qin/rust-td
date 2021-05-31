# rust-td

A demo for pure rust based td-shim.

It is derived from https://github.com/jyao1/edk2-staging/tree/TdShim/TdShimPkg.


## Known limitation
This package is only the sample code to show the concept. It does not have a full validation such as robustness functional test and fuzzing test. It does not meet the production quality yet. Any codes including the API definition, the libary and the drivers are subject to change.

## How to build

see [build.md](build.md)

## tdx-tdcall

tdx-tdcall impl two ways:

you can edit Cargo.toml -> features -> default to enable or disable. See

1. tdx call

```
default = []
```

2. tdx emulate

```
default = ["use_tdx_emulation"]
```


## Run
REF: https://github.com/tianocore/edk2-staging/tree/TDVF

```
./launch-rust-td.sh
```

## Code Contributions

1.  install [pre-commit](https://pre-commit.com/#install)
2.  run ```pre-commit install```
3.  when you run ```git commit```, pre-commit will do check-code things.

## Known limitation
This package is only the sample code to show the concept. It does not have a full validation such as robustness functional test and fuzzing test. It does not meet the production quality yet. Any codes including the API definition, the libary and the drivers are subject to change.
