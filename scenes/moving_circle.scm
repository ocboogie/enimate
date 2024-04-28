(define (main time)
  (object-tree 
    (list 
      (translate time 0.0 
                 (object-model (fill (color 255 0 0 255) (model (draw-circle 3.0))))))))

(define length 1.0)
