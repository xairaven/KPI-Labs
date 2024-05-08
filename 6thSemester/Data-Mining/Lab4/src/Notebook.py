from sklearn.datasets import load_wine
from scipy.cluster.vq import kmeans2, whiten

[data, target] = load_wine(return_X_y=True)
whitened = whiten(data)

start = [whitened[0],
         whitened[int(len(whitened)/2)],
         whitened[len(whitened)-1]]
[centroid, label] = kmeans2(whitened, start)

errors = 0.0
for i in range(len(label)):
    if target[i] != label[i]:
        errors += 1
        acc = ((len(data) - errors)
               /len(data))
        print (acc)



from sklearn.datasets import load_wine
from sklearn import tree

[data, target] = load_wine(return_X_y=True)

data_train = data[0:len(data)-2]
target_train = target[0:len(data)-2]

dtc = tree.DecisionTreeClassifier()
dtc = dtc.fit(data_train,
              target_train)
print (dtc.predict(
    data[len(data)-1]
    .reshape(1,-1)))


from sklearn.datasets import load_wine
from sklearn import svm

[data, target] = load_wine(return_X_y=True)
data_train = data[0:len(data)-2]
target_train = target[0:len(target)-2]

# clf = svm.LinearSVC()
clf = svm.LinearSVC(dual="auto")
clf.fit(data_train, target_train)
print (clf.predict(
    data[len(data)-1]
    .reshape(1,-1)))