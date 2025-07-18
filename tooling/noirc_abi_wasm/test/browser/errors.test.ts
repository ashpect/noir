import { expect } from '@esm-bundle/chai';
import initNoirAbi, { abiEncode } from '@noir-lang/noirc_abi';

beforeEach(async () => {
  await initNoirAbi();
});

it('errors when an integer input overflows', async () => {
  const { abi, inputs } = await import('../shared/uint_overflow');

  expect(() => abiEncode(abi, inputs)).to.throw(
    'The value passed for parameter `foo` does not match the specified type:\nValue Field(274877906944) does not fall within range of allowable values for a Integer { sign: Unsigned, width: 32 }',
  );
});

it('errors when passing a field in place of an array', async () => {
  const { abi, inputs } = await import('../shared/field_as_array');

  expect(() => abiEncode(abi, inputs)).to.throw(
    'cannot parse value `String("1")` into Array { length: 2, typ: Field }',
  );
});

it('errors when passing an array in place of a field', async () => {
  const { abi, inputs } = await import('../shared/array_as_field');

  expect(() => abiEncode(abi, inputs)).to.throw('cannot parse value `Array([String("1"), String("2")])` into Field');
});
