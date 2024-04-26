using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.UI;
public class CreditosFX : MonoBehaviour
{
    // reference to image object
    public Image shade;
    public GameObject creditos;

    public GameObject tractor;

    public float startAlpha;
    public float endAlpha = 0.9f;
    public float duration = 2.5f;

    public float initialY = -986.0f;


    

    // Start is called before the first frame update
    void Start()
    {
        shade = GameObject.Find("Shade").GetComponent<Image>();
        creditos = GameObject.Find("Creditos");

        StartCoroutine(FadeIn());
        StartCoroutine(MoveCredits());
    }

    IEnumerator FadeIn()
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime / duration;
            Color newColor = new Color(shade.color.r, shade.color.g, shade.color.b, Mathf.Lerp(startAlpha, endAlpha, t));
            shade.color = newColor;

            yield return null;
        }
    }

    IEnumerator MoveCredits()
    {
        float t = 0.0f;
        float duration2 = 5.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime / duration2;
            creditos.transform.localPosition = new Vector3(creditos.transform.localPosition.x, Mathf.Lerp(initialY, 0, t), creditos.transform.localPosition.z);
            
            yield return null;
        }
    }


    // Update is called once per frame
    void Update()
    {
        
    }
}
