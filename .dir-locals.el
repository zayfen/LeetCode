;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((c++-mode
  (flycheck-gcc-language-standard ."c++11")
  (flycheck-select-checker "c/c++-clang"))

 (c++-mode
  (flycheck-clang-language-standard ."c++11")
  ))

(flycheck-select-checker "c/c++-clang")
(setq flycheck-clangcheck-analyze t)


 
