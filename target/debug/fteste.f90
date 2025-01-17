module calculadora
  implicit none
contains

   ! Code removed by sparseFortran application on date 2025-01-17 as 11:03:40 

   function soma(a, b, resultado)
    real, intent(in) :: a, b
    real, intent(out) :: resultado
    resultado = a + b
  end function soma

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

program main
  use calculadora
  implicit none

  real :: a, b, resultado
  character(len=1) :: choice

  do
    print *, "Choose an operation: "
    print *, "(a) Add"
    print *, "(b) Subtract"
    print *, "(c) Multiply"
    print *, "(d) Divide"
    print *, "(e) Percentage"
    print *, "(f) Square Root"
    print *, "(q) Quit"
    read *, choice

    if (choice == 'q') exit

    if (choice /= 'f') then
      print *, "Enter first number: "
      read *, a
      print *, "Enter second number: "
      read *, b
    else
      print *, "Enter a number: "
      read *, a
    end if

    select case (choice)
    case ('a')
      call somar(a, b, resultado)
      print *, "Sum: ", resultado
    case ('b')
      call subtrair(a, b, resultado)
      print *, "Subtraction: ", resultado
    case ('c')
      call multiplicar(a, b, resultado)
      print *, "Multiplication: ", resultado
    case ('d')
      call dividir(a, b, resultado)
      print *, "Division: ", resultado
    case ('e')
      call aplicar_porcentagem(a, b, resultado)
      print *, "Percentage: ", resultado
    case ('f')
      call raiz_quadrada(a, resultado)
      print *, "Square Root: ", resultado
    case default
      print *, "Invalid choice!"
    end select

  end do

end program main
