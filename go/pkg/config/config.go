package config

import (
	"fmt"
	"os"
	"path"
)

type Operation = int

const (
	Print Operation = iota
	Add
	Delete
)

type Config struct {
	Args      []string
	Config    string
	Operation Operation
	Pwd       string
}

func getPwd(opts *Opts) (string, error) {
	if opts.Pwd != "" {
		return opts.Pwd, nil
	}
	return os.Getwd()
}

func getConfig(opts *Opts) (string, error) {
	if opts.Config != "" {
		return opts.Config, nil
	}
	config, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}
	return path.Join(config, "projector", "projector.json"), nil
}

func getOperation(opts *Opts) Operation {
	if len(opts.Args) == 0 {
		return Print
	}

	if opts.Args[0] == "add" {
		return Add
	}

	if opts.Args[0] == "del" {
		return Delete
	}

	return Print
}

func getArgs(opts *Opts) ([]string, error) {
	if len(opts.Args) == 0 {
		return []string{}, nil
	}
	operation := getOperation(opts)
	if operation == Delete {
		if len(opts.Args) != 2 {
			return []string{}, fmt.Errorf("Expected 1 argument but got %v", len(opts.Args)-1)
		}
		return opts.Args[1:], nil
	}

	if operation == Add {
		if len(opts.Args) != 3 {
			return []string{}, fmt.Errorf("Expected 2 arguments but got %v", len(opts.Args)-1)
		}
		return opts.Args[1:], nil
	}

	if len(opts.Args) > 1 {
		return []string{}, fmt.Errorf("Expected 1 or 0 arguments arguments but got ${opts.args.length}")
	}

	return opts.Args, nil
}

func NewConfig(opts *Opts) (*Config, error) {
	pwd, err := getPwd(opts)
	if err != nil {
		return nil, err
	}
	config, err := getConfig(opts)
	if err != nil {
		return nil, err
	}
	args, err := getArgs(opts)
	if err != nil {
		return nil, err
	}
	return &Config{args, config, getOperation(opts), pwd}, nil
}
