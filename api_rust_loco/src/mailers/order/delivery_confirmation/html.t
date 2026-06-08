<html>

<body>
  <h2>Your order has been delivered, {{name}}!</h2>
  <p>Your order <strong>{{order_number}}</strong> has been successfully delivered.</p>

  <h3>Delivery Details</h3>
  <table style="width:100%; border-collapse:collapse;">
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Order Number:</strong></td>
      <td>{{order_number}}</td>
    </tr>
    {% if delivered_at %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Delivered On:</strong></td>
      <td>{{delivered_at}}</td>
    </tr>
    {% endif %}
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
  </table>

  <p>We hope you enjoy your purchase! If you have any issues, please contact our support team.</p>
  <p>Best regards,<br>The Store Team</p>
</body>

</html>