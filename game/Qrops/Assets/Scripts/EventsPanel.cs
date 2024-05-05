using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

/* 
 * Esta clase se encarga de mostrar y ocultar un panel de eventos
 * cuando se hace clic en un botón.
 */

public class EventsPanel : MonoBehaviour
{
    public GameObject panel;
    
    // Esta función se llama cuando se hace clic en el botón
    // Muestra el panel de eventos
    public void HidePanel()
    {
        panel.SetActive(false);
    }
}
