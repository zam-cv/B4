using UnityEngine;

public class BotonInicio : MonoBehaviour
{
    public GameObject contenedorMaices; // Referencia al contenedor que contiene todos los maíces

    // Método que se llama cuando se hace clic en el botón
    public void OnClick()
    {
        // Recorre todos los hijos del contenedor
        foreach (Transform hijo in contenedorMaices.transform)
        {
            // Obtener el componente CrecimientoMaiz del hijo actual
            prueba crecimientoMaiz = hijo.GetComponent<prueba>();

            // Verificar si se encontró el componente CrecimientoMaiz
            if (crecimientoMaiz != null)
            {
                // Iniciar el crecimiento del maíz
                crecimientoMaiz.IniciarCrecimiento();
            }
            else
            {
                Debug.LogWarning("No se encontró el componente CrecimientoMaiz en un hijo del contenedor.");
            }
        }
    }
}
