(defun day2 ()
  "Solves day 2 of the advent of code.

Run in a buffer containing the puzzle input. It would write the
results to the end of the buffer."

  (interactive)

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

    (insert "\n"
            "Checksum: " (number-to-string checksum))))
