use arrow::array as arr;
use arrow::array::Array;

fn main() {
    let strs = arr::StringArray::from(vec![Some("foo"), Some("bar"), None, Some("barx")]);
    let strs = arr::StringArray::from(vec![Some("foo"), Some("bar"), None, Some("barx")]);
    let str_lens = calc_lens(&strs);
    assert_eq!(str_lens.len(), 4);
    assert_eq!(str_lens.values(), &[3, 3, 0, 4]);
    assert_eq!(str_lens.nulls().unwrap().null_count(), 1);
}

fn calc_lens(strs: &arr::StringArray) -> arr::UInt32Array {
    let mut arr_builder = arr::UInt32Builder::with_capacity(strs.len());
    strs.iter().for_each(|v| {
        if let Some(s) = v {
            arr_builder.append_value(s.len() as u32);
        } else {
            arr_builder.append_null();
        }
    });
    arr_builder.finish()
}
