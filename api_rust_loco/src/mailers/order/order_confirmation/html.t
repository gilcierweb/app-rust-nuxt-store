<html>

<body>
  <h2>Thank you for your order, {{name}}!</h2>
  <p>Your order <strong>{{order_number}}</strong> has been received and is being processed.</p>

  <h3>Order Summary</h3>
  <table style="width:100%; border-collapse:collapse;">
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Order Number:</strong></td>
      <td>{{order_number}}</td>
    </tr>
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Date:</strong></td>
      <td>{{order_date}}</td>
    </tr>
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Subtotal:</strong></td>
      <td>{{currency}} {{subtotal}}</td>
    </tr>
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Shipping:</strong></td>
      <td>{{currency}} {{shipping_amount}}</td>
    </tr>
    {% if discount_amount != "0.00" %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Discount:</strong></td>
      <td>-{{currency}} {{discount_amount}}</td>
    </tr>
    {% endif %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Total:</strong></td>
      <td><strong>{{currency}} {{total_amount}}</strong></td>
    </tr>
  </table>

  <h3>Items</h3>
  <table style="width:100%; border-collapse:collapse;">
    {% for item in items %}
    <tr style="border-bottom:1px solid #eee;">
      <td>{{item.name}} x {{item.quantity}}</td>
      <td>{{currency}} {{item.total}}</td>
    </tr>
    {% endfor %}
  </table>

  <p>You will receive another email when your order has shipped.</p>
  <p>Best regards,<br>The Store Team</p>
</body>

</html>