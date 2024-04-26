using UnityEngine;

public class ReiniciarMaiz : MonoBehaviour
{
    private GameObject contenedorMaices;
    public GameObject panelMensaje;  // Asegúrate de asignar esto desde el editor de Unity

    public void OnClick()
    {
        //switch dependiendo del nombre del panelMensaje
        // switch (panelMensaje.name)
        // {
        //     case "Panel Cosecha Maiz":
        //         contenedorMaices = GameObject.Find("Maices" + (ParcelaActual.instance.NumeroParcela - 1));
        //         break;
        //     case "Panel Cosecha Tomate":
        //         contenedorMaices = GameObject.Find("tomates" + (ParcelaActual.instance.NumeroParcela - 1));
        //         break;
        //     case "Panel Cosecha Cebada":
        //         contenedorMaices = GameObject.Find("cebadas" + (ParcelaActual.instance.NumeroParcela - 1));
        //         break;
        //     case "Panel Cosecha Caña":
        //         contenedorMaices = GameObject.Find("Cañas" + (ParcelaActual.instance.NumeroParcela - 1));
        //         break;
        // }

        //Dale el valor del primer elemento de la cola a contenedorMaices
        contenedorMaices = CultivosPlantados.instance.queueCultivos.Dequeue();
        // Desactiva el panel de mensajes
        panelMensaje.SetActive(false);

        // Reinicia cada maíz
        foreach (Transform hijo in contenedorMaices.transform)
        {
            prueba scriptMaiz = hijo.GetComponent<prueba>();
            if (scriptMaiz != null)
            {
                scriptMaiz.ResetCrecimiento();
            }
        }
    }
}
