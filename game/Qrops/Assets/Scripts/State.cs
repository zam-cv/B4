using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class State : MonoBehaviour
{
    [SerializeField]
    public TMP_Text scoreText;

    [SerializeField]
    public TMP_Text verqorText;

    [SerializeField]
    public TMP_Text coyoteText;

    [SerializeField]
    public TMP_Text cashText;

    public static State Instance { get; private set; }

    void Awake()
    {
        if (Instance == null)
        {
            Instance = this;
        }
        else
        {
            Destroy(gameObject);
        }
    }

    public void SetState(Player player)
    {
        Instance.scoreText.text = player.current_score.ToString();
        Instance.verqorText.text = player.balance_verqor.ToString();
        Instance.coyoteText.text = player.balance_coyote.ToString();
        Instance.cashText.text = player.balance_cash.ToString();
    }
}
