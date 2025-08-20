rust:
    RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+crt-static -C link-arg=-nostartfiles -C link-arg=-nodefaultlibs -C link-arg=-Wl,--gc-sections" \
    rustc --target x86_64-pc-windows-gnu \
    --cfg 'feature="utils"' \
    -C opt-level=z \
    -C panic=abort \
    -C debuginfo=0 \
    -C strip=symbols \
    -C codegen-units=1 \
    -C embed-bitcode=no \
    --emit=obj \
    src/lib.rs -o objects/rust_part.o

rust-beacon_api:
    RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+crt-static -C link-arg=-nostartfiles -C link-arg=-nodefaultlibs -C link-arg=-Wl,--gc-sections" \
    rustc --target x86_64-pc-windows-gnu \
    --cfg 'feature="utils"' \
    --cfg 'feature="beacon_api"' \
    -C opt-level=z \
    -C panic=abort \
    -C debuginfo=0 \
    -C strip=symbols \
    -C codegen-units=1 \
    -C embed-bitcode=no \
    --emit=obj \
    src/lib.rs -o objects/rust_part.o


rust-alloc:
    RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+crt-static -C link-arg=-nostartfiles -C link-arg=-nodefaultlibs -C link-arg=-Wl,--gc-sections" \
    rustc --target x86_64-pc-windows-gnu \
    --cfg 'feature="utils"' \
    --cfg 'feature="alloc"' \
    -C opt-level=z \
    -C panic=abort \
    -C debuginfo=0 \
    -C strip=symbols \
    -C codegen-units=1 \
    -C embed-bitcode=no \
    --emit=obj \
    src/lib.rs -o objects/rust_part.o

rust-all:
    RUSTFLAGS="-C target-cpu=x86-64 -C target-feature=+crt-static -C link-arg=-nostartfiles -C link-arg=-nodefaultlibs -C link-arg=-Wl,--gc-sections" \
    rustc --target x86_64-pc-windows-gnu \
    --cfg 'feature="utils"' \
    --cfg 'feature="alloc"' \
    --cfg 'feature="beacon_api"' \
    -C opt-level=z \
    -C panic=abort \
    -C debuginfo=0 \
    -C strip=symbols \
    -C codegen-units=1 \
    -C embed-bitcode=no \
    --emit=obj \
    src/lib.rs -o objects/rust_part.o

ld:
    x86_64-w64-mingw32-ld -r objects/rust_part.o objects/c_part.o -o objects/combined.o

copy: ld
    x86_64-w64-mingw32-objcopy \
        --remove-section=.drectve \
        --strip-symbol=@feat.00 \
        --remove-section=.bss \
        --strip-symbol=rust_begin_unwind \
        --strip-debug \
        objects/combined.o \
        bof.o



c:
    x86_64-w64-mingw32-gcc -c src/entry.c -o objects/c_part.o

c-utils:
    x86_64-w64-mingw32-gcc -DUTILS -c src/entry.c -o objects/c_part.o

c-all:
    x86_64-w64-mingw32-gcc -DUTILS -DBEACON_API -c src/entry.c -o objects/c_part.o


bof: bof-utils

bof-utils: rust c-utils copy view-bof

bof-beacon_api: rust-beacon_api c-all copy view-bof

bof-alloc: rust-alloc c-utils copy view-bof

bof-all: rust-all c-all copy view-bof

view-bof:
    objdump -t bof.o
