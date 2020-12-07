#include "electron.asm"

        * = $1910

        ;; 4-byte bitmask of questions that the current group has
        ;; asked so far
        FLAGS = $70
        ;; 4-byte counter of all question we found so far
        FLAG_SUM = $74
        ;; 1 bit to mark whether the last character was \n
        HAD_EOL = $78
        ;; temporary store for the question number
        QUESTION = $79
        ;; bit mask to check for this question
        QUESTION_BIT = $7A
        ;; scratch space for printing a number
        SCRATCH = $80

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

        ;; reset all variables to zero
        .(
        ldx #8
        lda #0
loop:   sta $70, x
        dex
        bpl loop
        .)

inputloop:
        .(
        jsr OSBGET              ; get next byte of input
        cmp #$fe                ; is it EOF?
        bcs done

        cmp #10                 ; is it end of line?
        bne noteol

        lda HAD_EOL             ; is this the second EOL?
        beq notendgroup

        ;; reset FLAGS to zero
        .(
        ldx #3
        lda #0
loop:   sta FLAGS, x
        dex
        bpl loop
        .)

notendgroup:
        lda #1                  ; mark that we had an EOL
        sta HAD_EOL
        bne inputloop           ; continue reading

noteol:
        ldx #0                  ; clear HAD_EOL
        stx HAD_EOL

        sec
        sbc #"a"                ; calculate question number
        bcc inputloop           ; ignore invalid question numbers
        cmp #32
        bcs inputloop

        sta QUESTION

        ;; calculate question bit
        .(
        and #7
        tax
        sec
        lda #0
loop:   rol
        dex
        bpl loop
        sta QUESTION_BIT
        .)

        ;; calculate byte number in x
        lda QUESTION
        lsr
        lsr
        lsr
        tax

        ;; has the question already been asked?
        lda FLAGS, x
        bit QUESTION_BIT
        bne inputloop

        ;; mark the question as asked
        ora QUESTION_BIT
        sta FLAGS, x

        ;; increment the question count
        sec
        lda FLAG_SUM
        adc #0
        sta FLAG_SUM
        lda FLAG_SUM + 1
        adc #0
        sta FLAG_SUM + 1
        lda FLAG_SUM + 2
        adc #0
        sta FLAG_SUM + 2
        lda FLAG_SUM + 3
        adc #0
        sta FLAG_SUM + 3

        bcc inputloop           ; continue input
done:
        lda #0
        jsr OSFIND              ; close the file
        jsr printsum
        lda #13
        jmp OSASCI
        .)

        ;; routine to print 4-byte number in FLAG_SUM. uses SCRATCH as
        ;; scratch space. corrupts number in FLAG_SUM
printsum:  
        .(
        ;; generate 10 digits
        ldy #9
loop:   jsr div_by_ten
        ;; remainder in a
        clc
        adc #"0"
        sta SCRATCH, y
        dey
        bpl loop
        .)
        .(
        iny
        ;; skip leading zeros
loop:   lda SCRATCH, y
        cmp #"0"
        bne done
        iny
        cpy #9 ; leave at least one digit
        bcc loop
done:
        .)
        .(
loop:   lda SCRATCH, y
        jsr $ffee
        iny
        cpy #10
        bcc loop
        .)
        rts

        ;; routine to divide 4-byte number in FLAG_SUM by 10
div_by_ten:     
        .( 
        ldx #32                 ; 32 bits
        lda #0
loop:   asl FLAG_SUM + 0
        rol FLAG_SUM + 1
        rol FLAG_SUM + 2
        rol FLAG_SUM + 3
        rol
        cmp #10
        bcc nobit
        sbc #10
        inc FLAG_SUM
nobit:  
        dex
        bne loop
        rts
        .)
        
filename:
        .byt "data", 13
