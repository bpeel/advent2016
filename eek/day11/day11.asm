#include "electron.asm"

        * = $1910

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
        XPOS = $70
        lda #0
        sta XPOS

        GRID_POS = $71          ; counter for where to store the next pos
        lda #<grid
        sta GRID_POS
        lda #>grid
        sta GRID_POS + 1

        FILENO = $73
        sty FILENO

        ldx #0                  ; offset from GRID_POS

loop:   
        jsr OSBGET              ; get the next byte of input
        cmp #10
        bne not_eol
        jsr finish_line
        jmp loop
not_eol:
        cmp #$fe
        bcs finish_data
        
        ;; if the byte is "L" (0x4C) we want 2, otherwise if "."
	;; (0x2e). Let’s just invert bit 1 and use that.
        eor #2
        and #2

        ;; are we on an odd X-pos?
        lsr XPOS
        bcc evenpos

        rol XPOS
        asl
        asl
        asl
        asl
        ora (GRID_POS, x)
        sta (GRID_POS, x)
        inc XPOS
        inc GRID_POS
        bne loop
        inc GRID_POS + 1
        bcc loop

evenpos:
        asl XPOS
        sta (GRID_POS, x)
        inc XPOS
        bne loop

finish_line:
        lsr XPOS
        bcc finish_even
        inc GRID_POS
        bne finish_even
        inc GRID_POS + 1
finish_even:    
        lda #64
        sec
        sbc XPOS
        beq done_line

        tay
        dey
        lda #0

        .(
line_loop:
        sta (GRID_POS, x)
        inc GRID_POS
        bne not_high
        inc GRID_POS + 1
not_high:       
        dey
        bpl line_loop
        .)

done_line:
        lda #0
        sta XPOS
        ldy FILENO
        rts

finish_data:
        jsr finish_line
        lda GRID_POS + 1
        cmp #>GRID_END
        bcc finish_data
        lda GRID_POS
        cmp #<GRID_END
        bcc finish_data
        .)

        lda #0
        jsr OSFIND              ; close the file

        rts
        
filename:
        .byt "data", 13

grid:
        ;; Space after the program for the grid of data

        ;; This is stored as a grid of 64x128 bytes where each byte
	;; stores two spaces on the boat. Each space takes up 4 bits
	;; where the lower two bits represent the current state and the
	;; upper bits represent the next state in the process of being
	;; calculated.

        ;; The values stored in the two bits for a space are:
        ;; 00 - No chair
        ;; 01 - (Not used)
        ;; 10 - Unoccupied chair
        ;; 11 - Occupied chair
        GRID_END = grid + 64 * 128
