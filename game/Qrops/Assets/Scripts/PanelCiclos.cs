using UnityEngine;

public class PanelCiclos : MonoBehaviour
{
    public GameObject panel; // Referencia al panel que quieres mostrar/ocultar

    // Función para activar o desactivar el panel
    public void TogglePanel()
    {
        // Comprueba si el panel está activo y lo desactiva, o viceversa
        panel.SetActive(!panel.activeSelf);
    }
}
