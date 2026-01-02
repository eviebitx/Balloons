ENTRY(kmain)

SECTIONS {
    . = 1M;

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .v_data : {
        *(.v_data)
        *(.v_text)
    }

    .bss : {
        *(.bss)
    }
}
