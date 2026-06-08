<html>

<body>
  <h2>Forgot something, {{name}}?</h2>
  <p>You left some items in your cart. Complete your purchase before they sell out!</p>

  <h3>Your Cart</h3>
  <table style="width:100%; border-collapse:collapse;">
    {% for item in items %}
    <tr style="border-bottom:1px solid #eee;">
      <td>{{item.name}} x {{item.quantity}}</td>
      <td>{{currency}} {{item.price}}</td>
    </tr>
    {% endfor %}
    <tr style="border-top:2px solid #333;">
      <td><strong>Total</strong></td>
      <td><strong>{{currency}} {{total}}</strong></td>
    </tr>
  </table>

  <p style="margin-top:20px;">
    <a href="{{cart_url}}" style="display:inline-block; padding:12px 24px; background-color:#4F46E5; color:#ffffff; text-decoration:none; border-radius:6px;">
      Complete Your Order
    </a>
  </p>

  <p>If you have any questions, feel free to reach out to our support team.</p>
  <p>Best regards,<br>The Store Team</p>
</body>

</html>