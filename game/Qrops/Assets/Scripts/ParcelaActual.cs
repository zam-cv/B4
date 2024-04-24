using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ParcelaActual : MonoBehaviour
{
    //Crea una variable publica la cual se pueda acceder desde otro script. Debe ser integer y llamarce NumeroParcela
    public int NumeroParcela;
    public static ParcelaActual instance;

    void Start()
    {
        //Si la instancia es nula, se le asigna el valor de esta clase
        if (instance == null) {
            instance = this;
        }
    }
    
    void OnMouseDown(){
        print(NumeroParcela);
    }
}
