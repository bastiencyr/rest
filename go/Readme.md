# To install dependencies, do :
go mod tidy

# Run the code:
gu run main.go

# reload the server at every change
go install github.com/cespare/reflex@latest
make watch

# to change the gopath, use go env
go env -w GOPATH="/home/you/.go"
