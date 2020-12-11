#include "electron.asm"

        * = $1910

        GRID_POS = $70          ; address of byte we’re looking at in the grid
        XPOS = $72
        YPOS = $73
        WIDTH = $75             ; size of the grid in the data
        HEIGHT = $76
        CHANGED = $77           ; has anything changed when we flip buffer?
        SEAT_COUNT = $78
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

        jsr set_palette

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

        .)

        .(
loop:   
        jsr step_simulation
        jsr flip_buffer
        lda CHANGED
        bne loop
        jmp count_seats
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

get_grid_pos:
        .(
        ;; sets X to 0, corrupts TEMP
        jsr get_grid_y
        lda #0
        sta TEMP
        lda XPOS
        and #$fe
        asl
        rol TEMP
        asl
        rol TEMP
        adc GRID_POS
        sta GRID_POS
        lda GRID_POS + 1
        adc TEMP
        sta GRID_POS + 1

        ldx #0
        rts
        .)

get_grid_value:
        .(
        ;; Sets bit 0 and 2 of the accumulator to the value of the
	;; grid at XPOS,YPOS
        jsr get_grid_pos
        lda #1
        bit XPOS
        beq even
        lda (GRID_POS, x)
        ror
        rts
even:   lda (GRID_POS, x)
        rts
        .)

is_occupied:
        .(
        ;; Is the position at XPOS,YPOS occupied
        ;; sets X to 0, corrupts TEMP
        ;; returns value in carry flag
        lda XPOS
        bmi invalid
        lda YPOS
        bmi invalid
        jsr get_grid_value
        ror
        rts
invalid:
        clc
        rts
        .)

count_neighbours:
        ;; count the neighbours around XPOS,YPOS.
        ;; sets X to 0. corrupts TEMP and TEMP+1
        ;; returns value in a
        .(
        lda #0
        sta TEMP+1
        dec XPOS
        dec YPOS

        .(
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        inc XPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        inc XPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        dec XPOS
        dec XPOS
        inc YPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        inc XPOS
        inc XPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        dec XPOS
        dec XPOS
        inc YPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        inc XPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        .(
        inc XPOS
        jsr is_occupied
        bcc not
        inc TEMP+1
not:    .)

        dec XPOS
        dec YPOS
        lda TEMP+1
        rts
        .)

step_simulation:
        .(
        lda #0
        sta XPOS
        sta YPOS

loop:
        jsr get_grid_value
        tax
        and #4                  ; is there a chair?
        beq done_pos

        txa
        and #1
        beq unoccupied

        jsr count_neighbours
        cmp #4
        bcc fill_seat
        ldy #$40
        bne set_value

unoccupied:
        jsr count_neighbours
        cmp #0
        beq fill_seat
        ldy #$40
        bne set_value

fill_seat:
        ldy #$50
set_value:
        jsr get_grid_pos
        lda XPOS
        ror
        tya
        bcc even
        asl
even:   ora (GRID_POS, x)
        sta (GRID_POS, x)        

done_pos:
        inc XPOS
        lda XPOS
        cmp WIDTH
        bcc loop

        lda #0
        sta XPOS
        inc YPOS
        lda YPOS
        cmp HEIGHT
        bcc loop

        rts
        .)

flip_buffer:
        .(
        lda #0
        sta CHANGED
        lda #<SCREEN_START
        sta GRID_POS
        lda #>SCREEN_START
        sta GRID_POS + 1
        ldy #0
loop:   lda (GRID_POS), y
        lsr
        lsr
        lsr
        lsr
        tax
        eor (GRID_POS), y
        and #$f
        beq nochange
        lda #1
        sta CHANGED
        txa
        sta (GRID_POS), y
nochange:       
        iny
        bne loop
        inc GRID_POS + 1
        bpl loop
        rts
        .)

count_seats:
        .(
        lda #0
        sta SEAT_COUNT
        sta SEAT_COUNT + 1
        lda #<SCREEN_START
        sta GRID_POS
        lda #>SCREEN_START
        sta GRID_POS + 1
        ldy #0

loop:   lda (GRID_POS), y

        ldx #0
        .(
        lsr
        bcc notbit
        inx
notbit: .)
        .(
        lsr
        bcc notbit
        inx
notbit: .)
        txa
        clc
        adc SEAT_COUNT
        sta SEAT_COUNT
        lda SEAT_COUNT + 1
        adc #0
        sta SEAT_COUNT + 1

        iny
        bne loop
        inc GRID_POS + 1
        bpl loop
        rts
        .)

set_palette:
        .(
        ldx #0
        ldy #0
iloop:  lda palette_vdu, y
        jsr OSWRCH
        iny
        cpy #6
        bne iloop
        ldy #0
        inx
        stx palette_vdu + 1
        txa
        and #3
        sta palette_vdu + 2
        cpx #16
        bne iloop
        rts
palette_vdu:
        .byt VDUPALETTE, 0, 0, 0, 0, 0
        .)

filename:
        .byt "data", 13

vdu_init:
        .byt VDUMODE, 2
        .byt 23, 1, 0, 0, 0, 0, 0, 0, 0, 0 // Disable cursor
        .byt 31, 0, 31 // move cursor to bottom of screen
        vdu_init_length = * - vdu_init
