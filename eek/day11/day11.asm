#include "electron.asm"

        * = $1910

        GRID_POS = $70          ; address of byte we’re looking at in the grid
        XPOS = $72
        YPOS = $73
        FILENO = $74
        WIDTH = $75             ; size of the grid in the data
        HEIGHT = $76
        TEMP = $80

        SCREEN_START = $3000

        ;; The screen memory is used directly to store the grid of seats.
        ;; The colours are:
        ;; 00 - No chair
        ;; 01 - (Not used)
        ;; 10 - Unoccupied chair
        ;; 11 - Occupied chair

        ;; initialise the display with VDU codes
        .(
        ldx #0
loop:   lda vdu_init, x
        jsr OSWRCH
        inx
        cpx #vdu_init_length
        bcc loop
        .)

        ldx #<filename
        ldy #>filename
        lda #$40                ; open
        jsr OSFIND

        .(
        cmp #0
        bne gotfile
        brk
        .byt 255, "Error opening file", 0
gotfile:        
        .)

        ;; store the file descriptor in Y so we can use it for OS calls
        tay
        ;; also store it for later in case we need to use Y for something
        sty FILENO

        ;; load the data
        .(
        lda #0
        sta XPOS
        sta YPOS
        lda #<SCREEN_START
        sta GRID_POS
        lda #>SCREEN_START
        sta GRID_POS + 1

        ldx #0                  ; set X to zero so we can use indirect addr

loop:   
        jsr OSBGET              ; get the next byte of input
        cmp #10
        beq eol
        cmp #$fe
        bcs finish_data
        
        ;; if the byte is "L" (0x4C) we want 4, otherwise if "."
        ;; (0x2e) we want 0. Let’s just grab an inverted bit 1 and put
        ;; it in bit 2
        eor #2
        and #2
        asl

        ;; are we on an odd X-pos?
        lsr XPOS
        bcc evenpos

        rol XPOS
        asl
        ora (GRID_POS, x)
        sta (GRID_POS, x)
        inc XPOS
        lda GRID_POS
        adc #8
        sta GRID_POS
        bcc loop
        inc GRID_POS + 1
        bcs loop                ; always jump to loop

evenpos:
        asl XPOS
        sta (GRID_POS, x)
        inc XPOS
        bne loop                ; always jump to loop

eol:
        lda XPOS
        sta WIDTH
        lda #0
        sta XPOS
        inc YPOS
        jsr get_grid_y
        jmp loop

finish_data:
        lda YPOS
        sta HEIGHT
        lda #0
        jsr OSFIND              ; close the file

        rts
        .)

get_grid_y:
        ;; Set GRID_POS to the start of the memory for the row at YPOS
        .(
        lda #0
        sta GRID_POS

        lda YPOS
        ;; divide by 8 then multiply by $200 = YPOS&~7 << 9
        and #$f8
        lsr
        lsr
        sta TEMP
        adc #>SCREEN_START
        sta GRID_POS + 1

        lda TEMP
        ;; divide by 16 to get high byte of (floor(YPOS / 8) * $80)
        lsr
        lsr
        ;; store the carry as the high bit of the low byte
        ror GRID_POS
        ;; add the calculated high byte
        adc GRID_POS + 1
        sta GRID_POS + 1

        ;; add the ypos % 8
        lda YPOS
        and #7
        adc GRID_POS
        sta GRID_POS
        rts
        .)
        
filename:
        .byt "data", 13

vdu_init:
        .byt VDUMODE, 2
        .byt 23, 1, 0, 0, 0, 0, 0, 0, 0, 0
        vdu_init_length = * - vdu_init
