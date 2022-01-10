# To install dependencies, do :
go mod tidy

# Run the code:
gu run main.go

# reload the server at every change
go install github.com/cespare/reflex@latest
make watch

# if reflex is not in your path, you need to add it
export PATH=$PATH:"path to your bin gopath"
The bin gopath is generally /home/yourusername/go/bin

# to change the gopath, use go env
go env -w GOPATH="/home/you/.go"
