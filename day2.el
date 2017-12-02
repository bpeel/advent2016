(defun day2-part1 ()
  (save-excursion
    (goto-char (point-min))

    (let ((checksum 0))
      (while (re-search-forward "[0-9]" (line-end-position) 't)
        (backward-char)
        (let ((min most-positive-fixnum)
              (max most-negative-fixnum))
          (while (re-search-forward "[0-9]+" (line-end-position) 't)
            (let ((num (string-to-number (match-string 0))))
              (if (< num min)
                  (setq min num))
              (if (> num max)
                  (setq max num))))
          (setq checksum (+ checksum (- max min)))
          (forward-line)))
      checksum)))

(defun day2-get-line ()
  (let (result)
    (while (re-search-forward "[0-9]+" (line-end-position) 't)
      (setq result (cons (string-to-number (match-string 0)) result)))
    result))

(defun day2-find-divisor (first others)
  (let ((result))
    (while (and others
                (let ((mx (max first (car others)))
                      (mn (min first (car others))))
                  (if (/= (% mx mn) 0)
                      t
                    (setq result (/ mx mn))
                    nil)))
      (setq others (cdr others)))
    result))

(defun day2-find-divisible-pair (line)
  (let ((result))
    (while (not (setq result (day2-find-divisor (car line) (cdr line))))
      (setq line (cdr line)))
    result))

(defun day2-part2 ()
  (save-excursion
    (goto-char (point-min))

    (let ((checksum 0))
      (while (re-search-forward "[0-9]" (line-end-position) 't)
        (backward-char)
        (setq checksum (+ checksum (day2-find-divisible-pair (day2-get-line))))
        (forward-line))
      checksum)))

(defun day2 ()
  "Solves day 2 of the advent of code.

Run in a buffer containing the puzzle input. It would write the
results to the end of the buffer."

  (interactive)

  (goto-char (point-max))
  (insert "\n"
          "Part 1: " (number-to-string (day2-part1)) "\n"
          "Part 2: " (number-to-string (day2-part2)) "\n"))
