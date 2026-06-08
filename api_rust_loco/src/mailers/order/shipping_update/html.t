<html>

<body>
  <h2>Your order has shipped, {{name}}!</h2>
  <p>Great news! Your order <strong>{{order_number}}</strong> is on its way to you.</p>

  <h3>Shipping Details</h3>
  <table style="width:100%; border-collapse:collapse;">
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Order Number:</strong></td>
      <td>{{order_number}}</td>
    </tr>
    {% if carrier %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Carrier:</strong></td>
      <td>{{carrier}}</td>
    </tr>
    {% endif %}
    {% if tracking_number %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Tracking Number:</strong></td>
      <td>{{tracking_number}}</td>
    </tr>
    {% endif %}
    {% if shipped_at %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Shipped On:</strong></td>
      <td>{{shipped_at}}</td>
    </tr>
    {% endif %}
  </table>

  {% if tracking_url %}
  <p><a href="{{tracking_url}}">Track your shipment</a></p>
  {% endif %}

  <p>You will receive another email when your order has been delivered.</p>
  <p>Best regards,<br>The Store Team</p>
</body>

</html>