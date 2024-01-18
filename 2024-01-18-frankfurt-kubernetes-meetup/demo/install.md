# Create Kind cluster
kind create cluster --name k8s-meetup --config kind.config

# Install calico CNI
kubectl create -f https://raw.githubusercontent.com/projectcalico/calico/v3.27.0/manifests/tigera-operator.yaml
kubectl create -f https://raw.githubusercontent.com/projectcalico/calico/v3.27.0/manifests/custom-resources.yaml

# Install Ingress
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml

# Emojivoto install
kubectl apply -k github.com/BuoyantIO/emojivoto/kustomize/deployment
