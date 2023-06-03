package projector_test

import (
	"testing"

	"github.com/invm/projector-cli-app/go/pkg/config"
	"github.com/invm/projector-cli-app/go/pkg/projector"
)

func getData() *projector.Data {
	return &projector.Data{
		Projector: map[string]map[string]string{
			"/": {
				"foo": "bar1",
				"fem": "is_great",
			},
			"/foo": {
				"foo": "bar2",
			},
			"/foo/bar": {
				"foo": "bar3",
			},
		},
	}
}

func getProjector(pwd string, data *projector.Data) *projector.Projector {
	return projector.CreateProjector(
		&config.Config{
			Args:      []string{},
			Operation: config.Print,
			Pwd:       pwd,
			Config:    "Testing",
		},
		data,
	)
}

func test(t *testing.T, projector projector.Projector, key, value string) {
	v, ok := projector.GetValue(key)
	if !ok {
		t.Errorf("expected to find value \"%v\"", value)
	}
	if value != v {
		t.Errorf("expected to find %v but found %v", value, v)
	}
}

func TestGetValue(t *testing.T) {
	projector := getProjector("/foo/bar", getData())
	test(t, *projector, "foo", "bar3")
	test(t, *projector, "fem", "is_great")
}

func TestSetValue(t *testing.T) {
	data := getData()
	projector := getProjector("/foo/bar", data)

	test(t, *projector, "foo", "bar3")
	projector.SetValue("foo", "bar4")

	test(t, *projector, "foo", "bar4")
	projector.SetValue("fem", "is_super_great")

	test(t, *projector, "fem", "is_super_great")
	projector = getProjector("/", data)
	test(t, *projector, "fem", "is_great")
}

func TestDeleteValue(t *testing.T) {
	data := getData()
	projector := getProjector("/foo/bar", data)

	test(t, *projector, "foo", "bar3")
	projector.DelValue("foo")
	test(t, *projector, "foo", "bar2")

	projector.DelValue("fem")
	test(t, *projector, "fem", "is_great")
}
