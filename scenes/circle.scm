(define (main time)
  (object-tree 
    (list 
      (translate -1.0 0.0 
                 (object-model (fill (color 255 0 0 255) (model (draw-circle 1.0)))))
      (translate 1.0 0.0 
                 (object-model (fill (color 255 0 0 255) (model (draw-circle 1.0))))))))

(define length 1.0)
