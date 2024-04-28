; (define fill? color?)
; (struct stroke (color width))
(struct component (builder props transform))
(struct component-group (children transform))
(struct anim (duration motion))
(define (dur animation) 
  (if (anim? animation) 
    (anim-duration animation) 
    0.0))

(define (build-object component)
  (if (component? component)
    (let* ((builder (component-builder component))
           (props (component-props component))
           (obj (apply builder props)))
      (apply-transform obj (component-transform component)))
    (error "build-object: not a component" component)))

; Doesn't check if props or transform are equal, but just ensures that
(define (component-equal? a b)
  (cond
    [(and (component? a) (component? b))
     ((equal? (component-builder a) (component-builder b)))]
    [else #f]))

; (define (update-component-object component object-table)
;   (if (component? component)
;     (let ((builder (component-builder component))
;           (props (component-props component)))
;      (hash-update object-table builder (builder props)))
;     (error "update-component: not a component" component)))
;
; (define (update-component component updater components object-table)
;   (cond
;     [(component? (car components))
;      (if (component-equal? (car components) component)
;        (let ((updated-component (updater (car component))))
;          (cons (cons updated-component (cdr components))
;                (update-component-object updated-component object-table)))
;        (cons component (update-props component prop-updater (cdr components) object-table)))]
;     [(component-group? (car components))
;      (let ((result (update-component component updater (component-group-children (car components)) object-table))
;            (updated-children (car result))
;            (object-table (cdr result))
;            (result (update-component component updater (cdr components) object-table))
;            (updated-components (car result))
;            (object-table (cdr result))
;            (components (cons (component-group updated-children (component-group-children (car components)))
;                              updated-components)))
;        (cons components object-table))]))

; (define (update-props c updater components object-table)
;   (update-component 
;     c 
;     (lambda (c) 
;       (component 
;         (component-builder c) 
;         (updater (component-props c)) 
;         (component-transform c)))
;     components 
;     object-table))

; (define (update-transform c updater components object-table)
;   (update-component 
;     c 
;     (lambda (c) 
;       (component 
;         (component-builder component) 
;         (component-props component) 
;         (updater (component-transform c))))
;     components 
;     object-table))

(define (add component)
  (lambda (components alpha) (cons component components)))

(define (move component x y)
  (lambda (components alpha) components))

(define (seq-aux total-duration animations components alpha)
  (if (or (null? animations) (< alpha 0.0))
    components
    (let* ((normalized-alpha (/ (dur (car animations)) total-duration))
          (adjusted-alpha (/ alpha normalized-alpha))
          (new-components (play (car animations) components adjusted-alpha))
          (new-alpha (- alpha normalized-alpha)))
      (seq-aux total-duration (cdr animations) new-components new-alpha))))

(define (seq animations)
  (let ((total-duration (apply + (map dur animations))))
    (anim total-duration (lambda (components alpha)
                          (seq-aux total-duration animations components alpha)))))

(define (circle x y r fill stroke)
         (component (lambda (r fill stroke)
                      (object-model (draw-circle r) fill stroke))
                    (list r fill stroke)
                    (transform-translate x y)))

(define c1 (circle -1.0 0.0 1.0 (color 255 0 0 255) '()))
(define c2 (circle 1.0 0.0 1.0 (color 0 255 0 255) '()))

(define scene (seq (list (anim 0.5 (add c1)) (add c2))))

(define (play animation components alpha)
  (if (anim? animation)
    (let ((motion (anim-motion animation)))
      (motion components alpha))
    (animation components alpha)))

(define (build-object-tree component-tree)
  (map build-object component-tree))
;
(define (main time)
  (object-tree
    (build-object-tree (play scene '() time))))

(define length (dur scene))
