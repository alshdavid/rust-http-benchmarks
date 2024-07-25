#!/bin/bash
go build -o dist/main ./src/main.go
exec ./dist/main $@
