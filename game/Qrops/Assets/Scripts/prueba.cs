using UnityEngine;

public class prueba : MonoBehaviour
{
    public Sprite[] etapasCrecimiento;
    private SpriteRenderer spriteRenderer;
    private int etapaActual = 0;
    private bool crecimientoIniciado = false;
    private float tiempoInicio;
    public GameObject panelMensaje;

    void Start()
    {
        spriteRenderer = GetComponent<SpriteRenderer>();
        spriteRenderer.sprite = null;
        panelMensaje.SetActive(false);
    }

    void Update()
    {
        if (crecimientoIniciado)
        {
            float tiempoTranscurrido = Time.time - tiempoInicio;
            int indiceEtapa = Mathf.FloorToInt(tiempoTranscurrido / 5f * etapasCrecimiento.Length);
            indiceEtapa = Mathf.Clamp(indiceEtapa, 0, etapasCrecimiento.Length - 1);
            spriteRenderer.sprite = etapasCrecimiento[indiceEtapa];
            if (indiceEtapa == etapasCrecimiento.Length - 1 && !panelMensaje.activeSelf)
            {
                crecimientoIniciado = false;
                panelMensaje.SetActive(true);
            }
        }
    }


    public void IniciarCrecimiento()
    {
        if (!crecimientoIniciado)
        {
            tiempoInicio = Time.time;
            etapaActual = 0;
            spriteRenderer.sprite = etapasCrecimiento[0];
            crecimientoIniciado = true;
            panelMensaje.SetActive(false);
        }
    }

    // Métodos públicos para manipular el estado desde otros scripts
    public void ResetCrecimiento()
    {
        spriteRenderer.sprite = null;
        crecimientoIniciado = false;
    }

    public bool EstaCreciendo()
    {
        return crecimientoIniciado;
    }
}
