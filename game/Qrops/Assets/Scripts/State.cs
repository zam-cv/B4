using System.Collections;
using System.Collections.Generic;
using UnityEngine;

/*
 * Clase que se encarga de actualizar el estado del jugador en la interfaz
 */
public class State : MonoBehaviour
{
    void Start()
    {
        Utils.Instance.SetState(Connection.Instance.player);
    }
}
