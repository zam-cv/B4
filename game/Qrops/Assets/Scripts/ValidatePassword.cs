using System;
using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
/* 
 * Esta clase se encarga de validar la contraseña ingresada por el usuario.
 */
public class ValidatePassword : MonoBehaviour
{
    private TMP_InputField password;
    private TMP_InputField confirm_password;

    private bool ValidPassword;
    
    // Inicia la validación de la contraseña
    public void Start()
    {
        // Get reference to TMP_InputField component named Password_input
        password = GameObject.Find("Password_input").GetComponent<TMP_InputField>();
        password.inputType = TMP_InputField.InputType.Password;
        // Get reference to TMP_InputField component named Confirm_password_input
        confirm_password = GameObject.Find("Password_check_input").GetComponent<TMP_InputField>();
        confirm_password.inputType = TMP_InputField.InputType.Password;
    }
    // Valida la contraseña ingresada por el usuario
    public void Validate()
    {
        // Check if password is valid
        ValidPassword = password.text == confirm_password.text;
        // Change text color to red if password is invalid in the confirm password field
        confirm_password.textComponent.color = ValidPassword ? Color.black : Color.red;
        print(ValidPassword ? "Password is valid" : "Password is invalid");
    }



    
}
