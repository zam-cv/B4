using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SelectParcela : MonoBehaviour
{
    void OnMouseDown(){
    Debug.Log("GameObject ha sido clickeado");
    //Si el gameobject se llama "Click" se accede a la variable NumeroParcela de ParcelaActual y se le da el valor de 1

    switch(gameObject.name){
        case "Click":
            ParcelaActual.instance.NumeroParcela = 1;
            break;
        case "Click (1)":
            ParcelaActual.instance.NumeroParcela = 2;
            break;
        case "Click (2)":
            ParcelaActual.instance.NumeroParcela = 3;
            break;
        case "Click (3)":
            ParcelaActual.instance.NumeroParcela = 4;
            break;
    }
}

}