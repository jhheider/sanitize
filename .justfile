# Show all scripts
default:
  just -l

# Generate coverage/lcov.info and upload to coveralls.io
coverage:
  cargo tarpaulin --engine llvm -o lcov --output-dir coverage --coveralls $COVERALLS_TOKEN

# Generate coverage/lcov.info
coverage-local:
  cargo tarpaulin --engine llvm -o lcov --output-dir coverage