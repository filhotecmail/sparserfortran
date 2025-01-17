module calculadora
  implicit none
contains

   ! Code removed by sparseFortran application on date 2025-01-17 as 11:03:40 

   

  function testerr45(a, b, resultado)
    real, intent(in) :: a, b
    real, intent(out) :: resultado
    resultado = a + b
  end function testerr45

  subroutine subtrair(a, b, resultado)
    real, intent(in) :: a, b
    real, intent(out) :: resultado
    resultado = a - b
  end subroutine subtrair

  subroutine multiplicar(a, b, resultado)
    real, intent(in) :: a, b
    real, intent(out) :: resultado
    resultado = a * b
  end subroutine multiplicar

  subroutine dividir(a, b, resultado)
    real, intent(in) :: a, b
    real, intent(out) :: resultado
    if (b /= 0.0) then
      resultado = a / b
    else
      print *, "Error: Division by zero!"
      resultado = 0.0
    end if
  end subroutine dividir

  subroutine aplicar_porcentagem(total, porcentagem, resultado)
    real, intent(in) :: total, porcentagem
    real, intent(out) :: resultado
    resultado = (porcentagem / 100.0) * total
  end subroutine aplicar_porcentagem

  subroutine raiz_quadrada(a, resultado)
    real, intent(in) :: a
    real, intent(out) :: resultado
    if (a >= 0.0) then
      resultado = sqrt(a)
    else
      print *, "Error: Square root of a negative number!"
      resultado = 0.0
    end if
  end subroutine raiz_quadrada

end module calculadora
