using UnityEngine;

public class ReiniciarMaiz : MonoBehaviour
{
    public GameObject contenedorMaices;
    public GameObject panelMensaje;  // Asegúrate de asignar esto desde el editor de Unity

    public void OnClick()
    {
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
