import { Operation, newConfig } from "../config";

test('Simple print all', () => {
  const config = newConfig({});
  expect(config.operation).toEqual(Operation.Print)
  expect(config.args).toEqual([])
})

test('Print key', () => {
  const config = newConfig({
    args: ["foo"]
  });
  expect(config.operation).toEqual(Operation.Print)
  expect(config.args).toEqual(["foo"])
})


test('Add key', () => {
  const config = newConfig({
    args: ["add", "foo", "bar"]
  });
  expect(config.operation).toEqual(Operation.Add)
  expect(config.args).toEqual(["foo", "bar"])
})

test('Delete key', () => {
  const config = newConfig({
    args: ["del", "foo"]
  });
  expect(config.operation).toEqual(Operation.Delete)
  expect(config.args).toEqual(["foo"])
})
