package config_test

import (
	"reflect"
	"testing"

	"github.com/invm/projector-cli-app/go/pkg/config"
)

func getConfig(t *testing.T, args []string) *config.Config {
	opts := &config.Opts{
		Args:   args,
		Config: "",
		Pwd:    "",
	}
	cfg, err := config.NewConfig(opts)
	if err != nil {
		t.Errorf("Expected to get not error %v", err)
	}
	return cfg
}

func testConfig(t *testing.T, args []string, expectedArgs []string, operation config.Operation) {
	cfg := getConfig(t, args)
	if operation == config.Add || operation == config.Delete {
		args = args[1:]
	}
	if !reflect.DeepEqual(cfg.Args, expectedArgs) {
		t.Errorf("Expected args %v bug got %v", args, cfg.Args)
	}
	if cfg.Operation != operation {
		t.Errorf("Expected operation %v but got %v", operation, cfg.Operation)
	}
}

func TestConfigPrint(t *testing.T) {
	testConfig(t, []string{}, []string{}, config.Print)
}

func TestConfigPrintAll(t *testing.T) {
	testConfig(t, []string{"foo"}, []string{"foo"}, config.Print)
}

func TestConfigAdd(t *testing.T) {
	testConfig(t, []string{"add", "foo", "bar"}, []string{"foo", "bar"}, config.Add)
}

func TestConfigDelete(t *testing.T) {
	testConfig(t, []string{"del", "foo"}, []string{"foo"}, config.Delete)
}
