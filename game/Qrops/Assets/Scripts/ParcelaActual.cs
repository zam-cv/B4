using System.Collections;
using System.Collections.Generic;
using UnityEngine;

/* 
 * Esta clase se encarga de manejar la parcela actual en la que se encuentra
 * el jugador.
 */

public class ParcelaActual : MonoBehaviour
{
    //Crea una variable publica la cual se pueda acceder desde otro script. Debe ser integer y llamarce NumeroParcela
    public int NumeroParcela;
    public static ParcelaActual instance;

    // Función que se llama al inicio del juego y asigna la instancia de esta clase
    void Start()
    {
        //Si la instancia es nula, se le asigna el valor de esta clase
        if (instance == null) {
            instance = this;
        }
    }
    
    // Se llama cuando se hace clic en la parcela, sirve para debuggear
    void OnMouseDown(){
        print(NumeroParcela);
    }
}
