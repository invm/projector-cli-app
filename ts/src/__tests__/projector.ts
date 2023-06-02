import { Operation } from "../config";
import { Projector } from "../projector";

function getData() {
  return {
    projector: {
      "/": {
        "foo": "bar1",
        "fem": "is_great"
      },
      "/foo": {
        "foo": "bar2"
      },
      "/foo/bar": {
        "foo": "bar3"
      }
    }
  }
}

function getProjector(pwd: string, data = getData()): Projector {
  return new Projector({
    args: [],
    operation: Operation.Print,
    pwd,
    config: "Testing"
  }, data)
}

test('getValueAll', () => {
  const projector = getProjector('/foo/bar');
  expect(projector.getValueAll()).toEqual({
    "fem": "is_great",
    "foo": "bar3"
  })
})

test('getValue', () => {
  const projector = getProjector('/foo/bar');
  expect(projector.getValue("foo")).toEqual("bar3")
})

test('getValue', () => {
  let projector = getProjector('/foo/bar');
  expect(projector.getValue("foo")).toEqual("bar3")
  projector = getProjector('/foo');
  expect(projector.getValue("foo")).toEqual("bar2")
  expect(projector.getValue("fem")).toEqual("is_great")
})

test('setValue', () => {
  const projector = getProjector('/foo/bar');
  projector.setValue("foo", "bar4")
  expect(projector.getValue("foo")).toEqual("bar4")
  projector.setValue("bar", "baz")
  expect(projector.getValue("bar")).toEqual("baz")
})

test('delValue', () => {
  const projector = getProjector('/foo/bar');
  projector.delValue("foo")
  expect(projector.getValue("foo")).toEqual("bar2")
})
