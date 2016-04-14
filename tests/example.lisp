(defn fac [x]
  (if (= x 0x01)
    1.0
    (* x (fac (- x 1)))))
