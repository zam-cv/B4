using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class Utils : MonoBehaviour
{
    public static Utils Instance { get; private set; }

    void Start()
    {
        if (Instance == null)
        {
            Instance = this;
        }
    }

    private void Awake()
    {
        if (Instance == null)
        {
            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        else
        {
            Destroy(gameObject);
        }
    }

    public void SetState(Player player)
    {
        GameObject scoreObject = GameObject.Find("score");
        TMP_Text scoreText = scoreObject.GetComponent<TMP_Text>();

        GameObject verqorObject = GameObject.Find("verqor");
        TMP_Text verqorText = verqorObject.GetComponent<TMP_Text>();

        GameObject coyoteObject = GameObject.Find("coyote");
        TMP_Text coyoteText = coyoteObject.GetComponent<TMP_Text>();

        GameObject cashObject = GameObject.Find("cash");
        TMP_Text cashText = cashObject.GetComponent<TMP_Text>();

        scoreText.text = player.current_score.ToString();
        verqorText.text = player.balance_verqor.ToString();
        coyoteText.text = player.balance_coyote.ToString();
        cashText.text = player.balance_cash.ToString();
    }

    public void SetTopPlayers(List<string> topPlayers)
    {
        GameObject topPlayersObject = GameObject.Find("topPlayers");
        TMP_Text topPlayersText = topPlayersObject.GetComponent<TMP_Text>();

        string topPlayersString = "";
        int i = 1;
        foreach (string player in topPlayers)
        {
            topPlayersString += i + ". " + player + "\n";
        }

        topPlayersText.text = topPlayersString;
    }
}
