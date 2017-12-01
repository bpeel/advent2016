(defun day4-calculate-checksum (str)
  (let ((i 0)
        (frequencies ()))
    (while (< i (length str))
      (let ((ch (aref str i)))
        (when (and (>= ch ?a) (<= ch ?z))
          (let ((slot (assoc ch frequencies)))
            (if slot
                (setcdr slot (1+ (cdr slot)))
              (push (cons ch 1) frequencies)))))
      (setq i (1+ i)))
    (setq frequencies (sort frequencies
                            (lambda (a b)
                              (if (= (cdr a) (cdr b))
                                  (< (car a) (car b))
                                (> (cdr a) (cdr b))))))
    (let ((i 0)
          (result (make-string 5 ? ))
          (l frequencies))
      (while (< i (length result))
        (aset result i (caar l))
        (setq l (cdr l))
        (setq i (1+ i)))
      result)))

(defun day4-decrypt (str key)
  (let ((result (make-string (length str) ? ))
        (i 0))
    (while (< i (length str))
      (let ((ch (aref str i)))
        (if (and (>= ch ?a) (<= ch ?z))
            (aset result i (+ (mod (+ (- ch ?a) key) 26) ?a))
          (aset result i ch)))
      (setq i (1+ i)))
    result))

(defun day4-solve ()
  "Solves day 4 of Advent of Code.

The current buffer should contain the data for the challenge. The
entire buffer is scanned for room descriptions (it doesn’t matter
what the region is). The result is displayed in the minibuffer."
  (interactive)

  (save-excursion
    (goto-char (point-min))
    (let ((sector-sum 0)
          (found-room nil))
      (while (re-search-forward (concat "^\\([a-z-]+?\\)-\\([0-9]+\\)"
                                        "\\[\\([a-z]+\\)\\]")
                                nil t)
        (let ((encrypted-room-name (match-string 1))
              (sector-id (string-to-number (match-string 2)))
              (expected-checksum (match-string 3)))
          (when (string-equal expected-checksum
                              (day4-calculate-checksum encrypted-room-name))
            (setq sector-sum (+ sector-sum sector-id))
            (when (string-equal (day4-decrypt encrypted-room-name sector-id)
                                "northpole-object-storage")
              (setq found-room sector-id)))))
      (unless found-room
        (error "No room found"))
      (message "Part 1: %i\nPart 2: %i"
               sector-sum
               found-room))))
