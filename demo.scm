; Make path drawer lib which will basically be an abstraction over 
; https://docs.rs/lyon_path/latest/lyon_path/builder/trait.PathBuilder.html

; draw-circle (-> x y r object)
; find-component-object (-> objects component object)
(struct point x y)
(struct rect min max)

bounding-box (-> object rect)

(define rect-min-x
  (point-x (rect-min rect)))
(define rect-max-x
  (point-x (rect-max rect)))
(define rect-min-y
  (point-y (rect-min rect)))
(define rect-max-y
  (point-y (rect-max rect)))
(define rect-center-x (rect)
  (let ((min-x (rect-min-x rect))
        (max-x (rect-max-x rect))
        (+ (/ (- max-x min-x) 2) min-x))))
(define rect-center-y (rect)
  (let ((min-y (rect-min-y rect))
        (max-y (rect-max-y rect))
        (+ (/ (- max-y min-y) 2) min-y))))

(define rect-center (rect)
  (point (rect-center-x rect) (rect-center-y rect)))
(define rect-left (rect)
  (point (rect-min-x rect) (rect-center-y rect)))
(define rect-right (rect)
  (point (rect-max-x rect) (rect-center-y rect)))
(define rect-top (rect)
  (point (rect-center-x rect) (rect-min-y rect)))
(define rect-bottom (rect)
  (point (rect-center-x rect) (rect-max-y rect)))

(define dynamic? (dyn) (procedure? dyn))

(define resolve-dynamic (objects dyn) (dyn objects))

(define resolve-arg (objects dyn)
  (if (dynamic? dyn)
    (resolve-dynamic dyn)
    (dyn)))

TODO: Make define-dynamic

(define bounding-box-of (component)
  (lambda (components)
    (bounding-box (find-component components component))))

(define left-of (component)
  (lambda (objects)
    (rect-left (bounding-box (find-component components component)))))

(define right-of (component)
  (lambda (objects)
    (rect-right (bounding-box (find-component components component)))))

(define alpha? (and/c (>=/c 0) (<=/c 1)))
(define motion? (-> objects? alpha? objects?))
(define duration? (>=/c 0))
(define animation? (cons/c duration? motion?))

(define/contract dur
                 (-> duration? motion? animation?)
                 cons)

Make component struct in rust
(struct component (builder args transform))

Make object struct in rust
(struct object ())

model-object (-> path transform fill stroke) 

(define build-component (component objects)
  (apply (component-builder component) objects (component-args component)))

; Make custom component define macro that allows specifying how arguments
; are interpolated

(define* (circle x y r #:optional fill #:optional stroke)
         (component (lambda (objects x y r)
                      (model-object (draw-circle 0 0 r) fill stroke))
                    '(x y r fill stroke)))

; component tree

(define (add component)
  (lambda (components)
    (cons component components)))

(define (move component pos)
  (lambda (components)
    (let ((pos (resolve-arg components pos)))
      (set-transform! component (translate pos)))))

(define c1 (circle 1 -1 0.5))
(define c2 (circle -1 -1 0.5))

(define (define-scene motion)
  (let ((components (component-tree)))
    (motion components)
    (components))))

(define-scene (add c1))

(define-scene 
  (seq (add c1)
       (add c2)
       (concurr (dur 1 (move c1 (point 1 1)))
                (dur 1 (move c2 (left-of c1))))))

animation tree -> component tree -> object tree

(define c1 (circle 0 0 5))

(seq (add c1)
     (dur 1 (move circle 2 2))
     (dur 1 (move circle 4 4)))

(define c2 (circle -1 0 5))
(define c3 (circle 0 -1 5))

(define )

(define (main time)
  (object 
    (model (draw-circle 3) (fill (color 0 0 0 1)) (stroke (color 1 1 1 1)))
    (transform)))

(define length 1)
