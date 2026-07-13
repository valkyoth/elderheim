# ELF Layout Contract

Status: active 0.13.0 security contract

`elderheim-format-elf` owns the checked layout model used before future ELF32
and ELF64 serialization. The crate remains `no_std` and does not emit an
executable in the current release.

## Address Domains

ELF file offsets and runtime virtual addresses are separate domains:

- `file_size` is the complete serialized file length;
- `text_offset` is the text segment's offset inside that file;
- `text_vaddr` is the text segment's runtime virtual address;
- `entry_vaddr` is the runtime entry address;
- `text_size` is checked independently in both domains.

No file offset is compared directly with a virtual address.

## Validation

`ImagePlan::validate` fails closed when:

- `text_offset + text_size` overflows;
- `text_vaddr + text_size` overflows;
- the text file range extends past `file_size`;
- the entry address is below the text virtual range;
- the entry address is at or beyond the exclusive text virtual end.

The file and virtual ends use `checked_add`. Saturating arithmetic is forbidden
for executable layout validation because it can turn an overflowing range into
an apparently valid one.

## Current Boundary

This contract validates one text range only. Program headers, segment
permissions, alignment, overlap detection, relocation placement, and exact ELF
serialization remain their own complete release-plan stops. Future layout
fields must identify their address domain explicitly.
